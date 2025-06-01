import { createAsyncThunk } from "@reduxjs/toolkit";
import { loginRequest, fetchMe } from "./authAPI";

export const login = createAsyncThunk(
  "auth/login",
  async (
    { email, password }: { email: string; password: string },
    { rejectWithValue }
  ) => {
    try {
      await loginRequest(email, password);
      const response = await fetchMe();
      return response.data;
    } catch (err: any) {
      return rejectWithValue(err.response?.data || err.message);
    }
  }
);

export const getMe = createAsyncThunk(
  "auth/getMe",
  async (_, { rejectWithValue }) => {
    try {
      const response = await fetchMe();
      return response.data();
    } catch (err: any) {
      return rejectWithValue(err.response?.data || err.message);
    }
  }
);
