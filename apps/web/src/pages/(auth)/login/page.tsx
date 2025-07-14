import { useEffect, useState } from "react";
import { login } from "../../../features/auth/authThunks";
import { useNavigate } from "react-router-dom";
import { useAppDispatch, useAppSelector } from "../../../app/hooks";

export default function Login() {
  const navigate = useNavigate();
  const dispatch = useAppDispatch();
  const { loading, error, user } = useAppSelector((state) => state.auth);

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await dispatch(login({ email, password }));
  };

  useEffect(() => {
    console.log("Redux user object:", user);
    if (user) {
      console.log("username: ", user.username);
      navigate("/channels");
    }
  }, [user, navigate]);

  return (
    <div className="">
      <div className="">
        <h1>Login</h1>
      </div>
      <form onSubmit={handleSubmit}>
        <input
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <button type="submit" disabled={loading}>
          Login
        </button>
        {error && <p style={{ color: "red" }}>{error}</p>}
      </form>
    </div>
  );
}
