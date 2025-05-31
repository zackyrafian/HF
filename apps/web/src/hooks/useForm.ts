import { useReducer } from "react";

type FormState = Record<string, string>;

type Action = {
  type: "CHANGE";
  payload: { name: string; value: string };
};

function reducer(state: FormState, action: Action): FormState {
  switch (action.type) {
    case "CHANGE":
      return { ...state, [action.payload.name]: action.payload.value };
    default:
      return state;
  }
}

export function useForm(initialState: FormState) {
  const [state, dispatch] = useReducer(reducer, initialState);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch({
      type: "CHANGE",
      payload: {
        name: e.target.name,
        value: e.target.value,
      },
    });
  };

  return { form: state, handleChange };
}
