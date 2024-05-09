/* Query 4
a trigger that updates the level of use after an uodate on the lending table*/

DELIMITER //
CREATE TRIGGER update_level_of_use
    AFTER UPDATE ON Lendings
    FOR EACH ROW
    BEGIN
        IF OLD.ReturnDate IS NULL AND NEW.ReturnDate IS NOT NULL THEN
            UPDATE
                Items
            SET
                LevelOfUse = (LevelOfUse + 0.10)
            WHERE
                ItemID = NEW.ItemID;
        END IF;

    END //
DELIMITER ;

