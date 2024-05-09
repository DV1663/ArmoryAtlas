# ==============This one is done================== #

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

# Implemented in python aswell
