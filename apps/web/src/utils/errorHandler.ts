import axios from "axios";

export function handleAxiosError(err: unknown): string {
  if (axios.isAxiosError(err)) {
    return (
      (err.response?.data as { message?: string })?.message ||
      err.message ||
      "Axios error"
    );
  }
  return "Unknown error occurred";
}
