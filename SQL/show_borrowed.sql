# Query 7
/*A procedure that shows the items that are borrowed by a user*/
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

drop procedure show_borrowed;
CALL show_borrowed('1');
