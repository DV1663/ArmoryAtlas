
# ======== Drop Tables ========
DROP TABLE IF EXISTS Lendings;
DROP TABLE IF EXISTS Users;
DROP TABLE IF EXISTS Items;
DROP TABLE IF EXISTS Products;


# ======== Drop Triggers ========
DROP TRIGGER IF EXISTS check_borrowed;
DROP TRIGGER IF EXISTS update_level_of_use;


# ======== Drop Procedures ========
DROP PROCEDURE IF EXISTS return_item;
DROP PROCEDURE IF EXISTS show_borrowed;


# ======== Drop Views ========
DROP VIEW IF EXISTS number_of_borrowes;


# ======== Drop Functions ========
DROP FUNCTION IF EXISTS in_stock_for_product;


