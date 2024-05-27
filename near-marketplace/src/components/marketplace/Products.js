import React, { useEffect, useState, useCallback } from "react";
import { toast } from "react-toastify";
import AddProduct from "./AddProduct";
import Product from "./Product";
import Loader from "../utils/Loader";
import { Row } from "react-bootstrap";
import { NotificationSuccess, NotificationError } from "../utils/Notification";
import {
  getProducts as getProductList,
  buyProduct,
  createProduct,
} from "../../utils/marketplace";

const Products = () => {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(false);

  // Fetch the list of products
  const getProducts = useCallback(async () => {
    // 제품 list fetch할 때 loading state true 설정 & getProductList 사용
    try {
      setLoading(true);
      setProducts(await getProductList());
    } catch (error) {
      console.log({ error });
    } finally {
      setLoading(false);
    }
  });

  // Create product from the product data received as a parameter
  // and fetch the products again by calling the above function
  const addProduct = async (data) => {
    try {
      setLoading(true);
      createProduct(data).then((resp) => {
        getProducts();
      });
      toast(<NotificationSuccess text="Product added successfully." />);
    } catch (error) {
      console.log({ error });
      toast(<NotificationError text="Failed to create a product." />);
    } finally {
      setLoading(false);
    }
  };

  // Buy the product by passing its id and price
  // and fetch the products again and display a(n) success/error message
  const buy = async (id, price) => {
    try {
      await buyProduct({
        id,
        price,
      }).then((resp) => getProducts());
      toast(<NotificationSuccess text="Product bought successfully" />);
    } catch (error) {
      toast(<NotificationError text="Failed to purchase product." />);
    } finally {
      setLoading(false);
    }
  };

  // `Products` component가 처음 화면에 나타날 때 제품 목록 가져오도록 함
  // 가져온 목록을 통해 products state 업데이트하고, component 다시 렌더링되어 최신 목록 표시
  useEffect(() => {
    getProducts();
  }, []);

  return (
    <>
    {!loading ? (
      <>
        <div className="d-flex justify-content-between align-items-center mb-4">
          <h1 className="fs-4 fw-bold mb-0">Street Food</h1>
          {/* Pass the `addProduct` function */}
          <AddProduct save={addProduct} /> 
        </div>
        <Row xs={1} sm={2} lg={3} className="g-3 mb-5 g-xl-4 g-xxl-5">
          {/* Map the list of products to the Product component */}
          {/* Pass the `_product`'s data and `buy` function */}
          {products.map((_product) => (
            <Product
              product={{
                ..._product,
              }}
              buy={buy}
            />
          ))}
        </Row>
      </>
     ) : (
      <Loader />
     )}
    </>
  );
};

export default Products;