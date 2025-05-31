import { useEffect, useState } from "react";
import { useForm } from "../../hooks/useForm";
import FormField from "../../components/forms/FormField";

const fields = [
  { name: "username", placeholder: "username" },
  { name: "email", placeholder: "Email", type: "email" },
  { name: "password", placeholder: "Password", type: "password" },
];
type User = {
  id: number;
  name: string;
  email: string;
};

export default function TestingPage() {
  const [user, setUser] = useState<User[]>([]);

  const { form, handleChange } = useForm({
    username: "",
    email: "",
    password: "",
  });

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchUser = async () => {
      const res = await fetch("http://127.0.0.1:8081/users");
      const data = await res.json();

      setUser(data);
    };
    fetchUser();
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(false);
    setError(null);

    try {
      const res = await fetch("http://127.0.0.1:8081/register", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(form),
      });

      if (!res.ok) {
        const contentType = res.headers.get("content-type");
        const errorText = contentType?.includes("application/json")
          ? (await res.json()).message
          : await res.text();

        throw new Error(errorText || "Something went wrong");
      }
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <div>
        <ul>
          {user.map((user) => (
            <li key={user.id} className="mb-1">
              {user.id} {user.name} - {user.email}
            </li>
          ))}
        </ul>
      </div>
      <div>
        <form
          onSubmit={handleSubmit}
          className="max-w-md mx-auto space-y-4 p-4"
        >
          <h1 className="text-2xl font-bold">Register</h1>

          {fields.map((field) => (
            <FormField
              key={field.name}
              name={field.name}
              type={field.type}
              placeholder={field.placeholder}
              value={form[field.name]}
              onChange={handleChange}
            />
          ))}

          {error && <p className="text-red-500">{error}</p>}

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700"
          >
            {loading ? "Registering..." : "Register"}
          </button>
        </form>
      </div>
    </div>
  );
}
