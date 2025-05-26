# defmodule Hf.WebSocketHandler do
#   @behaviour :cowboy_websocket

#   def init(req, state) do
#     {:cowboy_websocket, req, state}
#   end

#   def websocket_init(state) do
#     IO.puts("[Server] WebSocket connected")
#     {:ok, state}
#   end

#   def websocket_handle({:text, msg}, state) do
#     IO.puts("[Server] Receivec message: #{msg}")
#     {:reply, {:text, "Pesan diterima: #{msg}"}, state}
#   end

#   def websocket_handle(_, state), do: {:ok, state}
#   def websocket_info(_, state) , do: {:ok, state}
# end


defmodule Hf.WebSocketHandler do
  @behaviour :cowboy_websocket

  def init(req, _state) do
    {:cowboy_websocket, req, %{}}
  end

  def websocket_init(state) do
    {:ok, _} = Registry.register(Hf.Registry, self(), nil)
    {:ok, state}
  end

  def websocket_handle({:text, msg}, state) do
    IO.puts("[Server] Received: #{msg}")

    Registry.dispatch(Hf.Registry, nil, fn entries ->
      for {pid, _} <- entries, pid != self() do
        send(pid, {:send_msg, msg})
      end
    end)

    {:ok, state}
  end

  def websocket_info({:send_msg, msg}, state) do
    {:reply, {:text, "[Pesan baru] #{msg}"}, state}
  end

  def websocket_info(_info, state) do
    {:ok, state}
  end
end
