# Ref.Finance Stableswap helper

## Install
```bash
npm i @cawabunga/ref-finance-stableswap-math
```

## Usage
```javascript
import { getAmountOut } from '@cawabunga/ref-finance-stableswap-math';

const amountOut = getAmountOut(decimals, amounts, ampCoef, totalFee, tokenInIndex, tokenOutIndex, amountIn);
```
