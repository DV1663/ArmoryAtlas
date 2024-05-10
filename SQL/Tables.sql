
# ======== Table Users ========
CREATE TABLE Users (
  -- Primary key
	SSN VARCHAR(11) NOT NULL,
    
	-- Attributes
  Name VARCHAR(250) NOT NULL,

  PRIMARY KEY(SSN)
);


# ======== Table Products ========
CREATE TABLE Products (
  -- Primary key
	ProductID VARCHAR(16) NOT NULL,
    
	-- Attributes
  NameOfProduct VARCHAR(250) NOT NULL,
  Type VARCHAR(250) NOT NULL,
    
  PRIMARY KEY(ProductID)
);


# ======== Table Items ========
CREATE TABLE Items (
  -- Primary key
	ItemID BINARY(16) NOT NULL,

  -- Foreign Key    
  ProductID VARCHAR(16) NOT NULL,
    
	-- Attributes
  Size VARCHAR(4),
  LevelOfUse FLOAT NOT NULL,
    
  PRIMARY KEY(ItemID),
    
  CONSTRAINT FKs
	FOREIGN KEY(ProductID) REFERENCES Products(ProductID)
);


# ======== Table Lending ========
CREATE TABLE Lendings (
  -- Primary key
	LendingID BINARY(16) NOT NULL,
    
  -- Foreign Key
	SSN VARCHAR(11) NOT NULL,
  ItemID BINARY(16) NOT NULL,
    
	-- Attributes
	BorrowingDate DATE NOT NULL,
    ReturnDate DATE,

	PRIMARY KEY(LendingID),
    
  CONSTRAINT FK1
		FOREIGN KEY(SSN) REFERENCES Users(SSN),
	CONSTRAINT FK2
		FOREIGN KEY(ItemID) REFERENCES Items(ItemID)        
);



