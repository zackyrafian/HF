import { useEffect } from "react";
import { useAppDispatch, useAppSelector } from "../../app/hooks";
import { getMe } from "../../features/auth/authThunks";

export default function ChannelsPage() {
  const dispatch = useAppDispatch();
  const { user } = useAppSelector((state) => state.auth);

  useEffect(() => {
    dispatch(getMe());
  }, [dispatch]);

  return (
    <div>
      <h1>Channel</h1>
      {user ? <h2>{user.username}</h2> : <p>Loading user...</p>}
    </div>
  );
}
