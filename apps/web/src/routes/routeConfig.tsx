import type { RouteObject } from "react-router-dom";
import { HomePage, LoginPage, TestingPage } from "../features";
import HomeLayout from "../features/home/layout";
import AuthLayout from "../features/(auth)/layout";

export const routes: RouteObject[] = [
  {
    element: <HomeLayout />,
    children: [
      { path: "/", element: <HomePage /> },
      { path: "/testing", element: <TestingPage /> },
    ],
  },
  {
    element: <AuthLayout />,
    children: [{ path: "/login", element: <LoginPage /> }],
  },
];
