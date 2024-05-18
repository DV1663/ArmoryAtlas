import os
import mysql.connector
import toml
import uuid


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


class TotBorrowes:
    def __init__(self, ssn, name, tot_borrowes, curr_borrowes):
        self.ssn = ssn
        self.name = name
        self.tot_borrowes = tot_borrowes
        self.curr_borrowes = curr_borrowes

    def __repr__(self):
        return f"SSN: {self.ssn}, Name: {self.name}, Total Borrowes: {self.tot_borrowes}, Current Borrowes: {self.curr_borrowes}"


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
    The `DBHandler` class is responsible for interacting with the MySQL database
    that holds the Armory Atlas inventory data. It provides methods to establish
    a database connection and to perform queries such as retrieving available
    items.

    Attributes:
        db (mysql.connector.connection.MySQLConnection): The database connection object.
        cursor (mysql.connector.cursor.MySQLCursor): The cursor object for executing queries.

    Methods:
        __init__(self): Initializes the DBHandler instance, establishing a database connection.
        Get_items(self): Retrieves a list of available items from the database.
        Get_config(): Retrieves the database configuration from a .toml file.
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
        query = """
            SELECT * FROM Users ORDER BY RAND() LIMIT 1;
                    """

        self.cursor.execute(query)
        users = self.cursor.fetchall()
        users_list = [User(*users) for users in users]
        return users_list[0]

    def get_rand_item(self) -> Item:
        query = """
            SELECT BIN_TO_UUID(i.ItemID) as ItemID, 
                i.ProductID, 
                i.Size, 
                i.Quality
            FROM Items i
            WHERE i.ItemID NOT IN (
                SELECT ItemID FROM Lendings WHERE ReturnDate IS NULL
            ) limit 1;
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
        query = """
            SELECT * FROM Users;
                    """

        self.cursor.execute(query)
        users = self.cursor.fetchall()
        users_list = [User(*users) for users in users]
        return users_list

    def get_loans(self) -> list[AllBorrowed]:
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
        query = f"""
            CALL return_item(UUID_TO_BIN('{item_id}'));
        """

        try:
            # Execute the stored procedure
            results = self.cursor.execute(query, multi=True)

            # Fetch results to ensure procedure executed
            for result in results:
                if result.with_rows:
                    data = result.fetchall()
                    print("Procedure output:", data)
                else:
                    print("Procedure affected rows:", result.rowcount)

            # Commit the transaction
            self.db.commit()
            print("Item returned successfully")
        except Exception as e:
            print(f"An error occurred: {e}")

    def user_all_borrowed(self, ssn: str) -> list[AllBorrowed]:
        query = f"""
            select * from show_borrowed_view where SSN = ('{ssn}');
                    """

        self.cursor.execute(query)
        allborrowed = self.cursor.fetchall()
        allborrowed_list = [AllBorrowed(*allborrowed) for allborrowed in allborrowed]
        return allborrowed_list

    def number_of_borrowes(self) -> list[TotBorrowes]:
        query = """
            SELECT * FROM number_of_borrowes;
                    """

        self.cursor.execute(query)
        borrowes = self.cursor.fetchall()
        borrowes_list = [TotBorrowes(*borrowes) for borrowes in borrowes]
        return borrowes_list

    @staticmethod
    def get_config() -> dict:
        if os.name == 'nt':
            home_dir = os.getenv('USERPROFILE')
        else:
            home_dir = os.getenv('HOME')

        config_path = os.path.join(home_dir, '.config', 'armoryatlas', 'config.toml')
        with open(config_path, "r") as f:
            config = toml.load(f)
        return config

    def insert_loan(self, loan) -> None:
        borrowing_date = loan.borrowing_date.strftime('%Y-%m-%d') if loan.borrowing_date else None
        return_date = loan.return_date.strftime('%Y-%m-%d') if loan.return_date else None
        query = f"""
            INSERT INTO Lendings (LendingID, SSN, ItemID, BorrowingDate, ReturnDate) 
            VALUES (UUID_TO_BIN(UUID()), '{loan.user_id}', UUID_TO_BIN('{loan.item_id}'), '{borrowing_date}', %s);
        """

        try:
            self.cursor.execute(query, (return_date,))
            self.db.commit()  # Commit the transaction
        except mysql.connector.Error as err:
            self.db.rollback()  # Rollback the transaction in case of error
            raise err

    def insert_product(self, product) -> None:
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

    def test(self):
        query = """
            SELECT * from Users;
                    """

        self.cursor.execute(query)
        borrowes = self.cursor.fetchall()

        return borrowes

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
                        UPDATE
                            Items
                        SET
                            Quality = (Quality - 0.10)
                        WHERE
                            ItemID = NEW.ItemID;
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
        self._create_tables()
        self._create_triggers()
        self._create_functions()
        self._create_procedures()
        self._create_views()


if __name__ == "__main__":
    db = DBHandler()
    db.return_item("177bf0f5-1434-11ef-ad4b-00e04c0313ab")

