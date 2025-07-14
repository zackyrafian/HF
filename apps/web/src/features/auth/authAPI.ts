import axios from "axios";
import { API_URL } from "../../lib/axios";

export const loginRequest = (email: string, password: string) => {
  return axios.post(
    `${API_URL}/user/login`,
    { email, password },
    {
      withCredentials: true,
    }
  );
};

export const fetchMe = () => {
  return axios.get(`${API_URL}/user/me`, {
    withCredentials: true,
  });
};
