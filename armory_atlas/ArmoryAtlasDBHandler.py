import os

import mysql.connector
import toml


class DBHandler:

    def __init__(self):
        """
        Initializes a new instance of the DBHandler class.

        This constructor method establishes a connection to a MySQL database using the provided configuration.
        It creates a connection object using the mysql.connector library and the connection details retrieved from
        the get_config() method. The connection details include the host, user, password, and database name.
        The connection object is stored in the 'db' attribute of the class.

        Parameters:
            self

        Returns:
            None
        """
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

    def get_items(self) -> list:
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
        return items

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