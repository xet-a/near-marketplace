import React, { useCallback, useEffect, useState } from "react";
import { login } from "./utils/near";
import { getProducts } from "./utils/marketplace";
import './App.css';

function App() {
  const account = window.walletConnection.account();
  const [products, set_product] = useState([]);
  const fetchProducts = useCallback(async () => {
    if (account.accountId) {
      set_product(await getProducts());
    }
  });

  useEffect(() => {
    fetchProducts();
    // eslint-disable-next-line
  }, []);

  return (
    <>
      {account.accountId ? (
        products.forEach((product) => console.log(product))
      ) : (
        <button onClick={login}>CONNECT WALLET</button>
      )}
    </>
  );
}

export default App;
