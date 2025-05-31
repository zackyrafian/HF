import { BrowserRouter, useRoutes } from "react-router-dom";
import { routes } from "./routeConfig";

function AppRouterElement() {
  const element = useRoutes(routes);
  return element;
}

export default function AppRoutes() {
  return (
    <BrowserRouter>
      <AppRouterElement />
    </BrowserRouter>
  );
}
