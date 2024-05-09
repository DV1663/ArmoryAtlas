# ==============This one is done================== #
# Its implemented
/*
This query will only return the information for the product with the specified ID ('1') and the total count of items 
in stock for size 'L' for that product.
*/

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

/*KALLAR PÅ DEN OCH VISAR INFO*/
SELECT 
    p.ProductID,
    p.NameOfProduct AS ProductName,
    in_stock_for_product(p.ProductID, 'L') AS TotIn
FROM 
    Products p
where
	p.ProductID = '1';
    
drop function Instock;
