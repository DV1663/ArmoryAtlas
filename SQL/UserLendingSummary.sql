CREATE VIEW UserLendingSummary AS
SELECT
    u.SSN,
    u.Name,
    COUNT(l.LendingID) AS TotalLendings,
    SUM(CASE WHEN l.ReturnDate IS NULL THEN 1 ELSE 0 END) AS currLendings
FROM
    Users u
LEFT JOIN
    Lendings l ON u.SSN = l.SSN
GROUP BY
    u.SSN,
    u.Name
ORDER BY
    TotalLendings DESC;
    
SELECT * FROM UserLendingSummary;
