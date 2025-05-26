"use client";

import React from "react";
import { useRegisterForm } from "@/features/register/useRegisterForm";

export default function RegisterPage() {
  const {
    email,
    password,
    status,
    error,
    handleChangeEmail,
    handleChangePassword,
    handleSubmit,
  } = useRegisterForm();

  return (
    <div>
      <h1>Register</h1>
      <form onSubmit={handleSubmit}>
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={handleChangeEmail}
          required
        />
        <br />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={handleChangePassword}
          required
        />
        <br />
        <button type="submit" disabled={status === "loading"}>
          {status === "loading" ? "Registering..." : "Register"}
        </button>
      </form>

      {status === "succeeded" && (
        <p style={{ color: "green" }}>User registered successfully!</p>
      )}
      {status === "failed" && <p style={{ color: "red" }}>{error}</p>}
    </div>
  );
}
