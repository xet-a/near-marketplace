import React, { useCallback, useEffect, useState } from "react";
import { Container, Nav } from "react-bootstrap";
import { login, logout as destroy, accountBalance } from "./utils/near";
// import { getProducts } from "./utils/marketplace";
import Wallet from "./components/Wallet";
import Cover from "./components/utils/Cover";
import coverImg from "./assets/img/sandwich.jpg";
import './App.css';

const App = function AppWrapper() {
  const account = window.walletConnection.account();
  /* const [products, set_product] = useState([]);
  const fetchProducts = useCallback(async () => {
    if (account.accountId) {
      set_product(await getProducts());
    }
  });

  useEffect(() => {
    fetchProducts();
    // eslint-disable-next-line
  }, []);
  */
  const [balance, setBalance] = useState("0");
  // eslint-disable-next-line
  const getBalance = useCallback(async () => {
    if (account.accountId) {
      setBalance(await accountBalance());
    }
  });

  useEffect(() => {
    getBalance();
  }, [getBalance]);

  return (
    <>
      {/* <Notification /> */}
      {account.accountId ? (
        <Container fluid="md">
          <Nav className="justify-content-end pt-3 pb-5">
            <Nav.Item>
              <Wallet
                address={account.accountId}
                amount={balance}
                symbol="NEAR"
                destroy={destroy} // logout
              />
            </Nav.Item>
          </Nav>
          <main>{/* <Products /> */}</main>
        </Container>
        // products.forEach((product) => console.log(product))
      ) : (
        <Cover name="Street Food" login={login} coverImg={coverImg} />
        // <button onClick={login}>CONNECT WALLET</button>
      )}
    </>
  );
}

export default App;
