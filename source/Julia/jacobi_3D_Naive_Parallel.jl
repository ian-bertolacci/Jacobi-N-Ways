using ArgParse

s = ArgParseSettings()
@add_arg_table s begin
    "--N"
        help = "Edge size of computational grid"
        arg_type = Int64
        default = 1000
    "--T"
        help = "Time-steps"
        arg_type = Int64
        default = 100
end

parsed_args = parse_args(ARGS, s)

N = parsed_args["N"]
T = parsed_args["T"]

println( "N: $N\nT: $T" )

grid_r = zeros( Float64, N+2, N+2, N+2 )
grid_w = zeros( Float64, N+2, N+2, N+2 )

tic()
for t in 1:T
  @sync @parallel for x in 2:N+1
    for y in 2:N+1
      for z in 2:N+1
        grid_w[x,y,z] = (grid_r[x,y,z] + grid_r[x-1,y,z] + grid_r[x,y-1,z] + grid_r[x,y,z-1] +
                                         grid_r[x+1,y,z] + grid_r[x,y+1,z] + grid_r[x,y,z+1] ) / 7.0
      end
    end
  end

  local temp = grid_r
  grid_r = grid_w
  grid_w = temp
end

time = toq()
println( "Elapsed: ", time, "s" )
