import { LuCirclePlus } from "react-icons/lu";

export default function Sidebar() {
  return (
    <div className="flex flex-col min-w-8 bg-gray-700 text-white items-center p-2 gap-2">
      <h1>Sidebar</h1>
      <div className="border rounded-xl min-w-12 min-h-12">
        <button className="flex items-center justify-center w-10 h-10 bg-gray">
          <LuCirclePlus />
        </button>
      </div>
      <div className="border rounded-xl min-w-12 min-h-12">
        <button className="flex items-center justify-center w-10 h-10 bg-gray">
          <LuCirclePlus />
        </button>
      </div>
    </div>
  );
}
