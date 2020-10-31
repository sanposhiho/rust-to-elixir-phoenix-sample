defmodule RustPhxSampleWeb.PageController do
  use RustPhxSampleWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
