use ArmoryAtlas;

# ============================================================================================================== #
# ========================================== CREATE TABLES ===================================================== #
# ============================================================================================================== #

# =============================
# ======== Table Users ========
# =============================
CREATE TABLE IF NOT EXISTS Users (
    -- Primary key
    SSN VARCHAR(11) NOT NULL,

	-- Attributes
	Name VARCHAR(250) NOT NULL,

    PRIMARY KEY(SSN)
);


# ================================
# ======== Table Products ========
# ================================
CREATE TABLE IF NOT EXISTS Products (
    -- Primary key
	ProductID VARCHAR(16) NOT NULL,

	-- Attributes
    NameOfProduct VARCHAR(250) NOT NULL,
    Type VARCHAR(250) NOT NULL,

    PRIMARY KEY(ProductID)
);


# =============================
# ======== Table Items ========
# =============================
CREATE TABLE IF NOT EXISTS Items (
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
);


# ===============================
# ======== Table Lending ========
# ===============================
CREATE TABLE IF NOT EXISTS Lendings (
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
);

# ============================================================================================================== #
# ============================================================================================================== #





# ============================================================================================================== #
# ============================================ CREATE TIGGERS ================================================== #
# ============================================================================================================== #

# ====================================
# ============ Trigger 1 =============
# VERIFY THAT THE ITEM IS NOT BORROWED
# ====================================

DELIMITER //
CREATE TRIGGER check_borrowed
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
    END //
DELIMITER ;


# ================================
# ========== Trigger 2 ===========
# UPDATE CONDITION AFTER RETURN
# ================================


DELIMITER //
CREATE TRIGGER update_quality
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
    END //
DELIMITER ;

# ============================================================================================================== #
# ============================================================================================================== #





# ============================================================================================================== #
# ============================================ CREATE FUNCTIONS ================================================ #
# ============================================================================================================== #

# ====================================
# ============ Function 1 ============
#       IN STOCK FOR PRODUCT
# ====================================
DELIMITER //
CREATE FUNCTION in_stock_for_product (product CHAR(16), size CHAR(5))
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
END//
DELIMITER ;

# ============================================================================================================== #
# ============================================================================================================== #





# ============================================================================================================== #
# ============================================ CREATE PROCEDURES =============================================== #
# ============================================================================================================== #

# =============================================
# ============ Procedure 1 ====================
# UPDATE RETURN DATE TO TODAYS DATE AND CONFIRM
# =============================================

DELIMITER //
CREATE PROCEDURE return_item (IN LendID INT)
BEGIN
    UPDATE
        Lendings
    SET
        ReturnDate = CURDATE()
    WHERE
        LendingID = LendID;
    SELECT
        'Item returned' AS Message;
END //
DELIMITER ;


# ==========================================
# ============ Procedure 2 =================
# SHOW ALL ITEMS BORROWED BY A SPECIFIC USER
# ==========================================

DELIMITER //
CREATE PROCEDURE show_borrowed (IN SSN VARCHAR(15))
BEGIN
    SELECT
        l.LendingID,
        u.Name,
        i.ItemID,
        p.NameOfProduct,
        i.Size,
        l.BorrowingDate,
        l.ReturnDate
    FROM
        Users u
            JOIN
        Lendings l
            JOIN
        Items i ON l.ItemID = i.ItemID
            JOIN
        Products p ON i.ProductID = p.ProductID
    WHERE
        u.SSN = SSN
    ORDER BY
        i.ItemID;
END //
DELIMITER ;

# ============================================================================================================== #
# ============================================================================================================== #




# ============================================================================================================== #
# ============================================ CREATE VIEWS ==================================================== #
# ============================================================================================================== #

# ====================================
# ============ View 1 ================
#       NUMBER OF BORROWERS
# ====================================

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

# ============================================================================================================== #
# ============================================================================================================== #
