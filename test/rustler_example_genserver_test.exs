defmodule RustlerExampleGenServerTest do
  use ExUnit.Case

  test "init" do
    assert match?({:ok, %{ref: ref}} when is_reference(ref), RustlerExampleGenServer.init([]))
  end

  test "put and get" do
    {:ok, pid} = GenServer.start_link(RustlerExampleGenServer, [])

    assert is_nil(RustlerExampleGenServer.get(pid, "key"))
    assert RustlerExampleGenServer.put(pid, "key", "value")
    assert "value" == RustlerExampleGenServer.get(pid, "key")
  end

  test "init native" do
    assert match?(
             {:ok, %{ref: ref}} when is_reference(ref),
             RustlerExampleGenServer.Native.init([])
           )
  end

  test "put and get native" do
    {:ok, pid} = GenServer.start_link(RustlerExampleGenServer.Native, [])

    assert is_nil(RustlerExampleGenServer.get(pid, "key"))
    assert :ok == RustlerExampleGenServer.put(pid, "key", "value")
    assert "value" == RustlerExampleGenServer.get(pid, "key")
  end
end
