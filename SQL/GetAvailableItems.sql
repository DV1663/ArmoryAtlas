DELIMITER //
CREATE PROCEDURE GetAvailableItems()
BEGIN
    DECLARE total_items INT;
    DECLARE rand_item_id INT;

    -- Get total count of available items
    SELECT COUNT(*) INTO total_items FROM Items;

    -- Generate a random item ID
    SET rand_item_id = FLOOR(RAND() * total_items) + 1;

    -- Select a random available item
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
    LIMIT rand_item_id, 1;
END //
DELIMITER ;


drop procedure GetAvailableItems;

call GetAvailableItems;
