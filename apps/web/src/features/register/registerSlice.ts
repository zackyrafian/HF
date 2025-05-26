// registerSlice.ts
import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";

export const registerUser = createAsyncThunk(
  "register/registerUser",
  async (payload: { email: string; password: string }) => {
    const res = await fetch("http://localhost:8081/api/users", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });

    if (!res.ok) throw new Error("Failed to register");
    return await res.json();
  }
);

const registerSlice = createSlice({
  name: "register",
  initialState: {
    email: "",
    password: "",
    status: "idle",
    error: null as string | null,
  },
  reducers: {
    setEmail: (state, action) => {
      state.email = action.payload;
    },
    setPassword: (state, action) => {
      state.password = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(registerUser.pending, (state) => {
        state.status = "loading";
      })
      .addCase(registerUser.fulfilled, (state) => {
        state.status = "succeeded";
        state.error = null;
      })
      .addCase(registerUser.rejected, (state, action) => {
        state.status = "failed";
        state.error = action.error.message || "Failed to register";
      });
  },
});

export const { setEmail, setPassword } = registerSlice.actions;
export default registerSlice.reducer;
