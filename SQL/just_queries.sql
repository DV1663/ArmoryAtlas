# ============================================================================================================== #
# ============================================ CREATE QUERIES ================================================== #
# ============================================================================================================== #

# ====================================
# ============ Query 1 ===============
#       GET RANDOM AVAILABLE ITEM
# ====================================

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
GROUP BY
	rand(), i.ItemID
LIMIT 1;


# ====================================
# ============ Query 2 ===============
#       GET ALL ITEMS THAT EXISTS
# ====================================

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

# ============================================================================================================== #
# ============================================================================================================== #




