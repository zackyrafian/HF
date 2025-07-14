import type { RouteObject } from "react-router-dom";
import { HomePage, TestingPage } from "../features";
import HomeLayout from "../features/home/layout";
import { LoginPage, ChannelsPage } from "../pages";
import { ChannelsLayout } from "../pages/channels";

export const routes: RouteObject[] = [
  {
    element: <HomeLayout />,
    children: [
      { path: "/", element: <HomePage /> },
      { path: "/testing", element: <TestingPage /> },
    ],
  },
  {
    // element: <AuthLayout />,
    children: [{ path: "/login", element: <LoginPage /> }],
  },
  {
    element: <ChannelsLayout />,
    children: [{ path: "/channels", element: <ChannelsPage /> }],
  },
];
