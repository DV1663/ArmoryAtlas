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
