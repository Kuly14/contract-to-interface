// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface UniswapV3Pool {
	function snapshotCumulativesInside(int24 tickLower, int24 tickUpper) external view  returns ( int56 tickCumulativeInside, uint160 secondsPerLiquidityInsideX128, uint32 secondsInside );
	function observe(uint32[] calldata secondsAgos) external view  returns (int56[] memory tickCumulatives, uint160[] memory secondsPerLiquidityCumulativeX128s);
	function increaseObservationCardinalityNext(uint16 observationCardinalityNext) external ;
	function initialize(uint160 sqrtPriceX96) external;
	function mint( address recipient, int24 tickLower, int24 tickUpper, uint128 amount, bytes calldata data ) external  returns (uint256 amount0, uint256 amount1);
	function collect( address recipient, int24 tickLower, int24 tickUpper, uint128 amount0Requested, uint128 amount1Requested ) external  returns (uint128 amount0, uint128 amount1);
	function burn( int24 tickLower, int24 tickUpper, uint128 amount ) external  returns (uint256 amount0, uint256 amount1);
	function swap( address recipient, bool zeroForOne, int256 amountSpecified, uint160 sqrtPriceLimitX96, bytes calldata data ) external  returns (int256 amount0, int256 amount1);
	function flash( address recipient, uint256 amount0, uint256 amount1, bytes calldata data ) external ;
	function setFeeProtocol(uint8 feeProtocol0, uint8 feeProtocol1) external ;
	function collectProtocol( address recipient, uint128 amount0Requested, uint128 amount1Requested ) external  returns (uint128 amount0, uint128 amount1);
}
