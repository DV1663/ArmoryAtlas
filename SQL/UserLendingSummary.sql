CREATE VIEW UserLendingSummary AS
SELECT
    u.SSN,
    u.Name,
    COUNT(DISTINCT tot.LendingID) AS TotalLendings,
    COUNT(DISTINCT curr.LendingID) AS currLendings
FROM
    Users u
LEFT JOIN
    Lendings tot ON u.SSN = tot.SSN
LEFT JOIN
    Lendings curr ON u.SSN = curr.SSN 
AND
	curr.ReturnDate IS NULL
GROUP BY
    u.SSN,
    u.Name
ORDER BY
    TotalLendings DESC;
    
    
DROP view UserLendingSummary;
select * from UserLendingSummary;
