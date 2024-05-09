/* Query 5
 A procedure that updates the returndate to todays date so the trigger is used
   and confirmes it*/

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

CALL return_item(1);
