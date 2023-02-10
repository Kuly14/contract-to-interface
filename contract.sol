interface CidNFT {
	function tokenURI(uint256 _id) external view returns (string memory);
	function mint(bytes[] calldata _addList) external;
	function add( uint256 _cidNFTID, string calldata _subprotocolName, uint256 _key, uint256 _nftIDToAdd, AssociationType _type ) external;
	function remove( uint256 _cidNFTID, string calldata _subprotocolName, uint256 _key, uint256 _nftIDToRemove, AssociationType _type ) external;
	function getOrderedData( uint256 _cidNFTID, string calldata _subprotocolName, uint256 _key ) external view returns (uint256 subprotocolNFTID);
	function getPrimaryData(uint256 _cidNFTID, string calldata _subprotocolName) external view returns (uint256 subprotocolNFTID);
	function getActiveData(uint256 _cidNFTID, string calldata _subprotocolName) external view returns (uint256[] memory subprotocolNFTIDs);
	function activeDataIncludesNFT( uint256 _cidNFTID, string calldata _subprotocolName, uint256 _nftIDToCheck ) external view returns (bool nftIncluded);
	function onERC721Received( address, /*operator*/ address, /*from*/ uint256, /*id*/ bytes calldata /*data*/ ) external pure returns (bytes4);
	function testing() external pure returns(uint);
	function teseting2() external;
}
