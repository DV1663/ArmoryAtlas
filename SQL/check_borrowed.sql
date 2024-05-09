
# Query 6
/*A trigger that checks so a item isnt borrowed before the lending is made
  if it is, an error message is shown else a confirmation is made,
  the confirmation is shoen in python*/

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

/*Test the trigger*/
insert into Lendings (LendingID, SSN, ItemID, BorrowingDate, ReturnDate)
values (4, '1', 1 , '2024-04-02', NULL);

/*Test the trigger, where the item is unavalible*/
insert into Lendings (LendingID, SSN, ItemID, BorrowingDate, ReturnDate)
values (5, '1', 1 , '2024-04-02', NULL);

