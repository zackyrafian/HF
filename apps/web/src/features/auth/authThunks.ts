import { createAsyncThunk } from "@reduxjs/toolkit";
import { loginRequest, fetchMe } from "./authAPI";
import { handleAxiosError } from "../../utils/errorHandler";

export const login = createAsyncThunk(
  "auth/login",
  async (
    { email, password }: { email: string; password: string },
    { rejectWithValue }
  ) => {
    try {
      await loginRequest(email, password);
      const response = await fetchMe();
      console.log(response.data.username);
      return response.data;
    } catch (err: unknown) {
      return rejectWithValue(handleAxiosError(err));
    }
  }
);

// export const register = createAsyncThunk(
//   "auth/register",
//   async (
//     {
//       email,
//       password,
//       username,
//     }: { email: string; password: string; username: string },
//     { rejectWithValue }
//   ) => {
//     try {
//       await registerRequest(email, password, username);
//       const response = await registerRequest();
//       return response.data;
//     } catch (err: any) {
//       return rejectWithValue(err.response?.data || err.message);
//     }
//   }
// );

export const getMe = createAsyncThunk(
  "auth/getMe",
  async (_, { rejectWithValue }) => {
    try {
      const response = await fetchMe();
      console.log(response.data);
      return response.data;
    } catch (err: unknown) {
      return rejectWithValue(handleAxiosError(err));
    }
  }
);
