"use client";

import { useDispatch, useSelector } from "react-redux";
import type { AppDispatch, RootState } from "@/lib/store";
import { setEmail, setPassword, registerUser } from "./registerSlice";

export const useRegisterForm = () => {
  const dispatch = useDispatch<AppDispatch>();
  const { email, password, status, error } = useSelector(
    (state: RootState) => state.register
  );

  const handleChangeEmail = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(setEmail(e.target.value));
  };

  const handleChangePassword = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(setPassword(e.target.value));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(registerUser({ email, password }));
  };

  return {
    email,
    password,
    status,
    error,
    handleChangeEmail,
    handleChangePassword,
    handleSubmit,
  };
};
