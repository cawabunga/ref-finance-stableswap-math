/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} token_decimals
* @param {(bigint)[]} amounts
* @param {bigint} amp_factor
* @param {number} total_fee
* @param {number} token_in_index
* @param {number} token_out_index
* @param {bigint} amount_in
* @returns {bigint}
*/
export function getAmountOut(token_decimals: Uint8Array, amounts: (bigint)[], amp_factor: bigint, total_fee: number, token_in_index: number, token_out_index: number, amount_in: bigint): bigint;
