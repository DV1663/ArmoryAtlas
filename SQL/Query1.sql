/*
1. Query to Retrieve All Products in Stock
This query retrieves information about all products currently in stock, including their ID, name, type,size, and quantity available. 
It performs a JOIN operation between the Products and Items tables based on the ProductID, and groups them by ProductID and Size.
Filtering out products with a quantity of 0.

Vi får argumentera lite mer
*/


SELECT 
	i.ProductID,
    p.NameOfProduct AS ProductName,
    p.Type AS ProductType, 
    i.Quantity,
    i.Size AS SizeCount
FROM 
	Products p
JOIN 
	(SELECT ProductID, Size, count(*) as Quantity from Items group by ProductID, Size)
AS
    i ON p.ProductID = i.ProductID
WHERE
	i.Quantity > 0;

