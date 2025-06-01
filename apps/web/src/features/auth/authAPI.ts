import axios from "axios";

const API_URL = "http://localhost:8081";

export const loginRequest = (email: string, password: string) => {
  return axios.post(
    `${API_URL}/login`,
    { email, password },
    {
      withCredentials: true,
    }
  );
};

export const fetchMe = () => {
  return axios.get(`${API_URL}/users`, {
    withCredentials: true,
  });
};
