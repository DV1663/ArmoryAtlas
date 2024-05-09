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
    def __init__(self, product_id, size, tot_in):
        self.product_id = product_id
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


class Items:
    def __init__(self, item_id, product_id, size, level_of_use):
        self.item_id = uuid.UUID(bytes=item_id).__str__()
        self.product_id = product_id
        self.size = size
        self.level_of_use = level_of_use

    def __repr__(self):
        return f"Items({self.item_id}, {self.product_id}, {self.size}, {self.level_of_use})"


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
                        i ON p.ProductID = i.ProductID
                WHERE
                    i.Quantity > 0;
                    """

        self.cursor.execute(query)
        items = self.cursor.fetchall()
        item_list = [ItemProduct(*item) for item in items]
        return item_list

    """This query will only return the information for the product with the specified ID and the total count of items 
    in stock for a given size for that product."""

    def get_in_stock_size(self, product_id: str, size: str) -> list[InStockSize] | mysql.connector.Error:
        query = f"""
        SELECT 
                p.ProductID as product_id,
                p.NameOfProduct AS product_name,
                in_stock_for_product('{product_id}', '{size}') AS totIn
            FROM 
                Products p
            WHERE
                p.ProductID = '{product_id}';
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
            print("Error: ", err)
            return err

    """A view (number_of_borrowes) implwmented in mysql that we can use to get the total 
    number of borrowes for each user."""

    def number_of_borrowes(self) -> list[TotBorrowes]:
        query = """
            SELECT * FROM number_of_borrowes;
                    """

        self.cursor.execute(query)
        borrowes = self.cursor.fetchall()
        borrowes_list = [TotBorrowes(*borrowes) for borrowes in borrowes]
        return borrowes_list

    def get_rand_user(self) -> list[User]:
        query = """
            SELECT * FROM Users ORDER BY RAND() LIMIT 1;
                    """

        self.cursor.execute(query)
        users = self.cursor.fetchall()
        users_list = [User(*users) for users in users]
        return users_list

    def get_rand_item(self) -> list[Items]:
        query = """
            SELECT
                i.ItemID,
                i.ProductID,
                i.Size,
                i.LevelOfUse
            FROM
                Items AS i
            LEFT JOIN
                Lendings AS l ON i.ItemID = l.ItemID
            WHERE
                l.ItemID IS NULL OR l.ReturnDate < CURDATE()
            group by
                rand(), i.ItemID
            LIMIT 1;
        """

        # Execute the stored procedure
        self.cursor.execute(query)

        # Initialize the list to hold items
        items_list = []

        # Fetch all sets of results
        while True:
            # Fetch all rows from the current result set
            items = self.cursor.fetchall()
            items_list.extend([Items(*item) for item in items])

            # Check if there are more results
            if self.cursor.nextset() is None:
                break

        return items_list

    """ A procedure we call for that updates the returndate to the current date for the item with the specified ID. 
    a triiger is used in the background"""
    def return_item(self, lending_id: str):
        query = f"""
            CALL return_item('{lending_id}');
        """

        # Execute the stored procedure
        self.cursor.execute(query, multi=True)



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


if __name__ == "__main__":
    db = DBHandler()
    print(db.get_items())
    print(db.get_rand_item())
    print(db.get_in_stock_size("M240001-3708453", "XL"))
    print(db.number_of_borrowes())
    print(db.get_rand_user())
    db.return_item("232a3d13-05fd-11ef-ade3-00e04c0003ab") # This is a random UUID (tror den funkar, alltså funktionen)
