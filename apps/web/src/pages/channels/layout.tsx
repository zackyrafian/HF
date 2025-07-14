import { Outlet } from "react-router-dom";
import Sidebar from "./components/Sidebar";

export default function ChannelsLayout() {
  return (
    <div>
      <div className="h-screen flex ">
        <Sidebar />
        <div className="flex-1 flex flex-col min-w-0">
          <main className="flex overflow-auto bg-gray-500">
            <Outlet />
          </main>
        </div>
      </div>
    </div>
  );
}
