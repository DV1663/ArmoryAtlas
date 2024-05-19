from dataclasses import dataclass


def run_cli(args: list[str] | None) -> None:
    """
    Runs the cli program defined by the Rust library

    :param args: Should be `sys.argv` in most cases when running this from python
    :return:
        None
    """
    ...

@dataclass
class Item:
    item_id: str
    product_id: str
    size: str
    quality: float



@dataclass
class DetailedItem:
    product_id: str
    product_name: str
    product_type: str
    quantity: int
    size: str
    
@dataclass
class DetailedItems:
    items: list[DetailedItem]
    
@dataclass
class DetailedLoan:
    lending_id: str
    ssn: str
    name: str
    item_id: str
    product_name: str
    size: str
    borrow_date: str
    return_date: str | None = None

@dataclass
class DetailedLoans:
    loans: list[DetailedLoan]
    
@dataclass
class User:
    ssn: str
    name: str
    
@dataclass
class Users:
    users: list[User]
    
@dataclass
class Loans:
    leanding_id: str
    ssn: str
    name: str
    item_id: str
    product_name: str
    size: str
    borrow_date: str
    return_date: str | None = None
    
@dataclass
class InStockSize:
    product_id: str
    product_name: str
    size: str
    tot_in: int
    
@dataclass
class InStockSizes:
    sizes: list[InStockSize]

@dataclass
class DBHandler:
    def user_all_borrowed(self, ssn: str) -> list[DetailedLoan]:
        ...
    def get_items(self) -> list[DetailedItem]:
        ...

    def get_loans(self) -> list[DetailedLoan]:
        ...

    def get_users(self) -> list[User]:
        ...
    
    def get_rand_user(self) -> User:
        ...

    def get_rand_item(self) -> Item:
        ...
    
    def get_in_stock_size(self, product_id: str, size: str) -> InStockSizes:
        ...
    
    def insert_user(self, user: User) -> None:
        ...
    
    def insert_item(self, item: Item) -> None:
        ...
    
    def insert_loan(self, loan: Loans) -> None:
        ...
    
    def insert_product(self, product_id: str, product_name: str, product_type: str, quantity: int, size: str) -> None:
        ...
    
    def search_items(self, search_term: str) -> list[DetailedItem]:
        ...
    
    def drop_all(self) -> None:
        ...
    
    def create_all(self) -> None:
        ...
    ...

"""
get_items
get_in_stock_size
get_loans
get_rand_item
get_rand_user
insert_product
insert_item
insert_user
insert_loan
search_items
drop_all
create_all
get_users
return_item
user_all_borrowed
"""