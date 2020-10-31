defmodule RustPhxSampleWeb.SampleController do
  use RustPhxSampleWeb, :controller
  use Rustler, otp_app: :rust_phx_sample, crate: :rustphxsampleweb_samplecontroller

  def sample(conn, _params) do
    num = add(1, 2)

    render(conn, "sample.json", number: num)
  end

  def add(_a, _b), do: exit(:nif_not_loaded)
end
