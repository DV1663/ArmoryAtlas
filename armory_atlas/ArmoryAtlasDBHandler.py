import os
import mysql.connector
import toml
import uuid


class User:
    def __init__(self, ssn, name):
        self.ssn = ssn
        self.name = name

    def __repr__(self):
        return f"SSN: {self.ssn}, Name: {self.name}"


class Item:
    def __init__(self, item_id, product_id, size, quality):
        self.item_id = item_id
        self.product_id = product_id
        self.size = size
        self.quality = quality

    def __repr__(self):
        return f"Item({self.item_id}, {self.product_id}, {self.size}, {self.quality})"


def create_item_dict(item):
    return {
        "item_id": item.item_id,
        "product_id": item.product_id,
        "size": item.size,
        "quality": item.quality
    }


class ItemProduct:
    def __init__(self, product_id, product_name, product_type, quantity, size):
        self.product_id = product_id
        self.product_name = product_name
        self.product_type = product_type
        self.quantity = quantity
        self.size = size

    def __repr__(self):
        return (f"Product ID: {self.product_id}, Product Name: {self.product_name}, Product Type: {self.product_type}, "
                f"Quantity: {self.quantity}, Size: {self.size}")


class InStockSize:
    def __init__(self, product_id, product_name, size, tot_in):
        self.product_id = product_id
        self.product_name = product_name
        self.size = size
        self.tot_in = tot_in

    def __repr__(self):
        return f"Product ID: {self.product_id}, Size: {self.size}, In Stock: {self.tot_in}"


class NumberBorrow:
    def __init__(self, ssn, name, tot_borrowes, curr_borrowes):
        self.ssn = ssn
        self.name = name
        self.tot_borrowes = tot_borrowes
        self.curr_borrowes = curr_borrowes

    def __repr__(self):
        return f"SSN: {self.ssn}, Name: {self.name}, Total Borrowes: {self.tot_borrowes}, Current Borrowes: {self.curr_borrowes}"


class AllBorrowed:
    def __init__(self, lending_id, ssn, name, item_id, product_name, size, borrow_date, return_date):
        if return_date is not None:
            self.return_date = return_date.__str__()
        else:
            self.return_date = None

        self.lending_id = uuid.UUID(bytes=lending_id).__str__()
        self.ssn = ssn
        self.name = name
        self.item_id = uuid.UUID(bytes=item_id).__str__()
        self.product_name = product_name
        self.size = size
        self.borrow_date = borrow_date.__str__()

    def __repr__(self):
        return (
            f"AllBorrowed(lending_id: {type(self.lending_id).__name__} = {self.lending_id}, "
            f"ssn: {type(self.ssn).__name__} = {self.ssn}, "
            f"name: {type(self.name).__name__} = {self.name}, "
            f"item_id: {type(self.item_id).__name__} = {self.item_id}, "
            f"product_name: {type(self.product_name).__name__} = {self.product_name}, "
            f"size: {type(self.size).__name__} = {self.size}, "
            f"borrow_date: {type(self.borrow_date).__name__} = {self.borrow_date}, "
            f"return_date: {type(self.return_date).__name__} = {self.return_date})"
        )


class DBHandler:
    """
    A class for handling database operations on the Armory Atlas system.

    This class provides methods to interact with MySQL database to perform CRUD operations
    and execute specific business logic such as fetching random users or items,
    listing loans, and returning items.

    Attributes:
        db (mysql.connector.connection.MySQLConnection): The database connection object.
        cursor (mysql.connector.cursor.MySQLCursor): The cursor object for executing queries.

    Methods:
        __init__(self): Initializes a new DBHandler instance, establishes database connection using configuration settings.
        get_rand_user(self) -> User: Retrieves a random user from the Users table.
        get_rand_item(self) -> Item: Fetches a random item that is not currently lent out.
        get_users(self) -> list[User]: Gets a list of all users from the Users table.
        get_loans(self) -> list[AllBorrowed]: Retrieves a detailed list of all loans, including user and item information.
        get_items(self) -> list[ItemProduct]: Gets a list of items, along with product details and available quantity.
        get_in_stock_size(self, product_id: str, size: str) -> list[InStockSize]: Gets the stock count for a specific product ID and size.
        return_item(self, item_id: str) -> None: Executes a stored procedure to return an item and update the Lendings table.
        user_all_borrowed(self, ssn: str) -> list[AllBorrowed]: Retrieves all borrowed items for a specific user.
        number_of_borrows(self) -> int: Retrieves the number of borrows for each user, both current and total.
        get_config() -> dict: Retrieves the configuration settings for the database connection.
        insert_user(self, user) -> None: Inserts a new user into the Users table.
        insert_item(self, item) -> None: Inserts a new item into the Items table.
        insert_loan(self, loan) -> None: Inserts a new lending into the Lendings table.
        insert_product(self, product) -> None: Inserts a new product into the Products table.
        search_items(self, product_id: str, size: str) -> list[Item]: Searches for items in the Items table based on product ID and size.
        drop_all(self) -> None: Drops all tables, triggers, functions, procedures, and views in the database.
        create_all(self) -> None: Creates all tables, triggers, functions, procedures, and views in the database.
    """

    def __init__(self):
        config = self.get_config()
        host = config.get("host")
        # split host at ':'
        host, port = host.split(":")
        # keyring.get_password("armoryatlas", f"{config.get('user')}@{host}:{port}")

        self.db = mysql.connector.connect(
            host=host,
            user=config.get("user"),
            password=config.get("password"),
            database=config.get("database"),
            port=int(port),
        )

        self.cursor = self.db.cursor()

    def get_rand_user(self) -> User:
        """
        Retrieves a random user from the Users table.

        :return:
            A User object representing the random user fetched from the database.
        """
        query = """
            SELECT * FROM Users ORDER BY RAND() LIMIT 1;
                    """

        self.cursor.execute(query)
        users = self.cursor.fetchall()
        users_list = [User(*users) for users in users]
        return users_list[0]

    def get_rand_item(self) -> Item:
        """
        Fetches a random item not currently lent out.

        :return:
            An Item object representing the random item fetched from the database.

        :raise Exception: If no item is available to borrow.
        :raise mysql.connector.Error: If there is an error while executing the database query.
        """

        query = """
            SELECT BIN_TO_UUID(i.ItemID) as ItemID, 
                i.ProductID, 
                i.Size, 
                i.Quality
            FROM Items i
            WHERE i.ItemID NOT IN (
                SELECT ItemID FROM Lendings WHERE ReturnDate IS NULL
            ) order by rand() limit 1;
        """

        try:
            self.cursor.execute(query)
            items = self.cursor.fetchone()
        except mysql.connector.Error as err:
            raise err
        try:
            item = Item(*items)
        except Exception:
            raise Exception("No item available to borrow!")
        return item

    def get_users(self) -> list[User]:
        """
        Retrieves all users from the Users table and returns a list of User objects.

        :return:
            A list of User objects representing all users in the database.
        """
        query = """
            SELECT * FROM Users;
                    """

        self.cursor.execute(query)
        users = self.cursor.fetchall()
        users_list = [User(*users) for users in users]
        return users_list

    def get_loans(self) -> list[AllBorrowed]:
        """
        Retrieves all loans from the Lendings table and returns a list of AllBorrowed objects.

        :return:
            A list of AllBorrowed objects representing all loans in the database.

        """
        query = """
            SELECT l.LendingID, l.SSN, u.Name, l.ItemID, p.NameOfProduct, i.Size, l.BorrowingDate, l.ReturnDate
            FROM Lendings l
            JOIN Users u ON l.SSN = u.SSN
            JOIN Items i ON l.ItemID = i.ItemID
            JOIN Products p ON i.ProductID = p.ProductID

            ORDER BY 
                u.SSN,
                CASE WHEN l.ReturnDate IS NULL THEN 0 ELSE 1 END,
                l.BorrowingDate DESC,
                l.ReturnDate DESC;
                    """

        self.cursor.execute(query)
        loans = self.cursor.fetchall()
        loans_list = [AllBorrowed(*loan) for loan in loans]
        return loans_list

    def get_items(self) -> list[ItemProduct]:
        """
        Retrieves a list of items along with their product details and quantity.

        :return:
            A list of ItemProduct objects representing the items fetched from the database.
        """
        query = """
            SELECT
                    i.ProductID as product_id,
                    p.NameOfProduct AS product_name,
                    p.Type AS product_type,
                    i.Quantity as quantity,
                    i.Size AS size
                FROM
                    Products p
                        JOIN
                    (SELECT ProductID, Size, count(*) as Quantity from Items group by ProductID, Size)
                        AS
                        i ON p.ProductID = i.ProductID;
                    """

        self.cursor.execute(query)
        items = self.cursor.fetchall()
        item_list = [ItemProduct(*item) for item in items]
        return item_list

    def get_in_stock_size(self, product_id: str, size: str) -> list[InStockSize]:
        """
        Retrieves the in-stock size information for a specific product and size.

        :param product_id: The ID of the product as a string.
        :param size: The size of the product as a string.

        :return:
            A list of InStockSize objects containing information about the product's availability in the specified size.

        :raise mysql.connector.Error: If there is an error while executing the database query.
        """
        query = f"""
            SELECT 
                p.ProductID as product_id,
                p.NameOfProduct AS product_name,
                i.Size AS size,
                in_stock_for_product('{product_id}', '{size}') AS totIn
            FROM 
                Products p
            JOIN 
                Items i ON p.ProductID = i.ProductID
            WHERE
                p.ProductID = '{product_id}' AND i.Size = '{size}'
            LIMIT 1;
            """
        try:
            # Execute query with multi=True
            results = self.cursor.execute(query, multi=True)

            # Iterate through all result sets
            size_stock_list = []
            for result in results:
                if result.with_rows:  # Check if the result has rows
                    # Fetch all rows from the current result set
                    size_stock = result.fetchall()
                    size_stock_list.extend([InStockSize(*stock) for stock in size_stock])
            return size_stock_list

        except mysql.connector.Error as err:
            raise err

    def return_item(self, item_id: str):
        """
        Returns an item to the inventory.

        :param item_id: The ID of the item to be returned as a string.
        :return:
            None

        :raise mysql.connector.Error: If there is an error while executing the database query.
        """
        query = f"""
            CALL return_item(UUID_TO_BIN('{item_id}'));
        """

        try:
            # Execute the stored procedure
            results = self.cursor.execute(query, multi=True)

            # Fetch results to ensure procedure executed
            for result in results:
                if result.with_rows:
                    _ = result.fetchall()
                else:
                    print("Procedure affected rows:", result.rowcount)

            # Commit the transaction
            self.db.commit()
        except Exception as e:
            raise e

    def user_all_borrowed(self, ssn: str) -> list[AllBorrowed]:
        """
        Retrieves all borrowed items for a specific user.

        :param ssn: The SSN of the user as a string.
        :return:
            A list of AllBorrowed objects containing information about the user's borrowed items.
        """
        query = f"""
            select * from show_borrowed_view where SSN = ('{ssn}');
                    """

        self.cursor.execute(query)
        allborrowed = self.cursor.fetchall()
        allborrowed_list = [AllBorrowed(*allborrowed) for allborrowed in allborrowed]
        return allborrowed_list

    def number_of_borrowes(self) -> list[NumberBorrow]:
        """
        Retrieves the total number of borrowes for each user.

        :return:
            A list of TotBorrowes objects containing information about the total number of borrowes for each user.
        """
        query = """
            SELECT * FROM number_of_borrowes;
                    """

        self.cursor.execute(query)
        borrowes = self.cursor.fetchall()
        borrowes_list = [NumberBorrow(*borrowes) for borrowes in borrowes]
        return borrowes_list

    @staticmethod
    def get_config() -> dict:
        """
        Retrieves the Armory Atlas config file.

        :return:
            A dictionary containing the Armory Atlas config.
        """
        if os.name == 'nt':
            home_dir = os.getenv('USERPROFILE')
        else:
            home_dir = os.getenv('HOME')

        config_path = os.path.join(home_dir, '.config', 'armoryatlas', 'config.toml')
        with open(config_path, "r") as f:
            config = toml.load(f)
        return config

    def insert_loan(self, loan) -> None:
        """
        Inserts a loan into the database.

        :param loan: The loan object to be inserted.
        :return:
            None
        :raise mysql.connector.Error: If there is an error while executing the database query.
        """
        borrowing_date = loan.borrowing_date.strftime('%Y-%m-%d') if loan.borrowing_date else None
        return_date = loan.return_date.strftime('%Y-%m-%d') if loan.return_date else None
        query = f"""
            INSERT INTO Lendings (LendingID, SSN, ItemID, BorrowingDate, ReturnDate) 
            VALUES (UUID_TO_BIN(UUID()), '{loan.ssn}', UUID_TO_BIN('{loan.item_id}'), '{borrowing_date}', %s);
        """

        try:
            self.cursor.execute(query, (return_date,))
            self.db.commit()  # Commit the transaction
        except mysql.connector.Error as err:
            self.db.rollback()  # Rollback the transaction in case of error
            raise err

    def insert_product(self, product) -> None:
        """
        Inserts a product into the database.

        :param product: The product object to be inserted.
        :return:
            None
        :raise mysql.connector.Error: If there is an error while executing the database query.
        """
        query = f"""
            INSERT INTO Products (ProductID, NameOfProduct, Type) VALUES ('{product.product_id}', '{product.product_name}', '{product.product_type}')
            """

        try:
            self.cursor.execute(query)
            self.db.commit()  # Commit the transaction
        except mysql.connector.Error as err:
            self.db.rollback()  # Rollback the transaction in case of error
            raise err

    def insert_item(self, item) -> None:
        """
        Inserts an item into the database.

        :param item: The item object to be inserted.
        :return:
            None
        :raise mysql.connector.Error: If there is an error while executing the database query.
        """
        query = f"""
            INSERT INTO Items (ItemID, ProductID, Size, Quality) VALUES (UUID_TO_BIN(UUID()), \"{item.product_id}\", \"{item.size}\", {item.quality})
        """

        try:
            self.cursor.execute(query)
            self.db.commit()  # Commit the transaction
        except mysql.connector.Error as err:
            self.db.rollback()  # Rollback the transaction in case of error
            raise err

    def insert_user(self, user) -> None:
        """
        Inserts a user into the database.

        :param user: The user object to be inserted.
        :return:
            None
        :raise mysql.connector.Error: If there is an error while executing the database query.
        """
        query = f"""
            INSERT INTO Users (SSN, Name) VALUES (\"{user.ssn}\", \"{user.name}\")
        """

        try:
            self.cursor.execute(query)
            self.db.commit()  # Commit the transaction
        except mysql.connector.Error as err:
            self.db.rollback()  # Rollback the transaction in case of error
            raise err

    def search_items(self, search_param: str) -> list[ItemProduct]:
        """
        Searches for items in the database.
        It will get a list of items that have the search parameter in their name, type or size.

        :param search_param: The search parameter.
        :return:
            A list of items that have the search parameter in their name, type or size.
        """
        query = f"""
            SELECT
                i.ProductID as product_id,
                p.NameOfProduct AS product_name,
                p.Type AS product_type,
                i.Quantity as quantity,
                i.Size AS size
            FROM
                Products p
                    JOIN
                (SELECT ProductID, Size, count(*) as Quantity from Items group by ProductID, Size)
                    AS
                    i ON p.ProductID = i.ProductID
            WHERE
                (
                    p.NameOfProduct LIKE '%{search_param}%' OR
                    p.Type LIKE '%{search_param}%' OR
                    i.Size LIKE '%{search_param}%'
                )
            ORDER BY
                p.NameOfProduct
        """

        self.cursor.execute(query)
        items = self.cursor.fetchall()
        items_list = [ItemProduct(*item) for item in items]
        return items_list

    def _drop_tables(self):
        queries = [
            """
                DROP TABLE IF EXISTS Lendings;
            """,
            """
                DROP TABLE IF EXISTS Items;
            """,
            """
                DROP TABLE IF EXISTS Products;
            """,
            """
                DROP TABLE IF EXISTS Users;
            """,
        ]

        for query in queries:
            self.cursor.execute(query)

    def _drop_triggers(self):
        queries = [
            """
                DROP TRIGGER IF EXISTS check_borrowed;
            """,
            """
                DROP TRIGGER IF EXISTS update_level_of_use;
            """
        ]

        for query in queries:
            self.cursor.execute(query)

    def _drop_procedures(self):
        queries = [
            """
                DROP PROCEDURE IF EXISTS return_item;
            """,
            """
                DROP PROCEDURE IF EXISTS show_borrowed;
            """
        ]

        for query in queries:
            self.cursor.execute(query)

    def _drop_views(self):
        queries = [
            """
                DROP VIEW IF EXISTS show_borrowed_view;
            """,
            """
                DROP VIEW IF EXISTS number_of_borrowes;
            """
        ]

        for query in queries:
            self.cursor.execute(query)

    def _drop_functions(self):
        query = """
            DROP FUNCTION IF EXISTS in_stock_for_product;
        """

        self.cursor.execute(query)
        self.cursor.fetchall()

    def drop_all(self):
        """
        Drops all the tables in the database.

        :return:
            None
        """
        self._drop_tables()
        self._drop_triggers()
        self._drop_procedures()
        self._drop_views()
        self._drop_functions()

    def _create_tables(self):
        queries = [
            """CREATE TABLE IF NOT EXISTS Users (
                -- Primary key
                SSN VARCHAR(11) NOT NULL,
            
                -- Attributes
                Name VARCHAR(250) NOT NULL,
            
                PRIMARY KEY(SSN)
            );""",
            """CREATE TABLE IF NOT EXISTS Products (
                -- Primary key
                ProductID VARCHAR(16) NOT NULL,
            
                -- Attributes
                NameOfProduct VARCHAR(250) NOT NULL,
                Type VARCHAR(250) NOT NULL,
            
                PRIMARY KEY(ProductID)
            );""",
            """CREATE TABLE IF NOT EXISTS Items (
                -- Primary key
                ItemID BINARY(16) NOT NULL,
            
                -- Foreign Key
                ProductID VARCHAR(16) NOT NULL,
            
                -- Attributes
                Size VARCHAR(4),
                Quality FLOAT NOT NULL,
            
                PRIMARY KEY(ItemID),
            
                CONSTRAINT FKs
                    FOREIGN KEY(ProductID) REFERENCES Products(ProductID)
            );""",
            """CREATE TABLE IF NOT EXISTS Lendings (
                -- Primary key
                LendingID BINARY(16) NOT NULL,
            
                -- Foreign Key
                SSN VARCHAR(11) NOT NULL,
                ItemID BINARY(16) NOT NULL,
            
                -- Attributes
                BorrowingDate DATE NOT NULL,
                ReturnDate DATE,
            
                PRIMARY KEY(LendingID),
            
                CONSTRAINT FK1
                    FOREIGN KEY(SSN) REFERENCES Users(SSN),
                CONSTRAINT FK2
                    FOREIGN KEY(ItemID) REFERENCES Items(ItemID)
            );""",

        ]

        for query in queries:
            self.cursor.execute(query)

    def _create_triggers(self):
        queries = [
            """
            CREATE TRIGGER IF NOT EXISTS check_borrowed
                BEFORE INSERT ON Lendings
                FOR EACH ROW
                BEGIN
                    DECLARE borrowed INT;
            
                    SELECT
                        COUNT(*)
                    INTO
                        borrowed
                    FROM
                        Lendings
                    WHERE
                        ItemID = NEW.ItemID
                    AND
                        ReturnDate IS NULL;
            
                    IF borrowed > 0 THEN
                        SIGNAL SQLSTATE '45000'
                        SET MESSAGE_TEXT = 'Item is already borrowed';
                    END IF;
                END;
            """,
            """
            CREATE TRIGGER IF NOT EXISTS update_quality
            AFTER UPDATE ON Lendings
            FOR EACH ROW
            BEGIN
                IF OLD.ReturnDate IS NULL AND NEW.ReturnDate IS NOT NULL THEN
                    UPDATE Items
                    SET Quality = (Quality - 0.10)
                    WHERE ItemID = NEW.ItemID;
                END IF;
            END;
            """
        ]

        for query in queries:
            self.cursor.execute(query)

    def _create_functions(self):
        query = """
            CREATE FUNCTION IF NOT EXISTS in_stock_for_product (product CHAR(16), size CHAR(5))
            RETURNS INT
            DETERMINISTIC
            BEGIN
                DECLARE NrIn INT;
            
                SELECT COUNT(*) INTO NrIn
                FROM
                    Items i
                LEFT JOIN
                    Lendings l
                ON
                    i.ItemID = l.ItemID AND l.ReturnDate IS NULL
                WHERE
                    i.ProductID = product
                AND
                    (i.Size = size OR (i.Size IS NULL AND size IS NULL))
                AND
                    l.ItemID IS NULL;
            
                RETURN NrIn;
            END;
        """

        for result in self.cursor.execute(query, multi=True):
            pass  # Ensure we iterate through all results

    def _create_procedures(self):
        query = """
            CREATE PROCEDURE IF NOT EXISTS return_item(IN item_id BINARY(16))
            BEGIN
                UPDATE
                    Lendings
                SET
                    ReturnDate = CURDATE()
                WHERE
                    ItemID = item_id
                AND
                    ReturnDate IS NULL;
            END;
        """

        for result in self.cursor.execute(query, multi=True):
            pass  # Ensure we iterate through all results

    def _create_views(self):
        query = """
            CREATE VIEW number_of_borrowes AS
            SELECT
                u.SSN,
                u.Name,
                COUNT(DISTINCT tot.LendingID) AS TotalLendings,
                COUNT(DISTINCT curr.LendingID) AS currLendings
            FROM
                Users u
            LEFT JOIN
                Lendings tot ON u.SSN = tot.SSN
            LEFT JOIN
                Lendings curr ON u.SSN = curr.SSN
            AND
                curr.ReturnDate IS NULL
            GROUP BY
                u.SSN,
                u.Name
            ORDER BY
                TotalLendings DESC;
            
            CREATE VIEW show_borrowed_view AS
                SELECT
                    l.LendingID,
                    u.SSN,
                    u.Name,
                    i.ItemID,
                    p.NameOfProduct,
                    i.Size,
                    l.BorrowingDate,
                    l.ReturnDate
                FROM
                    Users u
                JOIN
                    Lendings l ON u.SSN = l.SSN
                JOIN
                    Items i ON l.ItemID = i.ItemID
                JOIN
                    Products p ON i.ProductID = p.ProductID;
        """

        for result in self.cursor.execute(query, multi=True):
            pass  # Ensure we iterate through all results

    def create_all(self):
        """
        Create all tables, triggers, functions, procedures and views

        :return:
            None
        """
        self._create_tables()
        self._create_triggers()
        self._create_functions()
        self._create_procedures()
        self._create_views()

    def test(self):
        query = """
            SELECT * FROM ArmoryAtlas.Items where Quality <= 0.10;
        """

        self.cursor.execute(query)
        items = self.cursor.fetchall()
        print(items)


if __name__ == "__main__":
    db = DBHandler()
    db.test()

