import { BrowserRouter, Routes, Route } from "react-router-dom";
import { HomePage, LoginPage } from "../pages";

export default function AppRoutes() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />}></Route>
        <Route path="/login" element={<LoginPage />}></Route>
      </Routes>
    </BrowserRouter>
  );
}
