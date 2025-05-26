defmodule HF.ChatTest do
  use ExUnit.Case
  doctest HF.Chat

  test "greets the world" do
    assert HF.Chat.hello() == :world
  end
end
