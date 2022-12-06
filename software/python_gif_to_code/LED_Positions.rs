const stbd : [[f32;2];256] = [
    [ 172.05889299999998, 117.226393],
    [ 173.1, 112.39999999999999],
    [ 172.358893, 121.626393],
    [ 171.758893, 126.42639299999999],
    [ 171.958893, 131.126393],
    [ 177.658893, 128.426393],
    [ 176.2, 132.6],
    [ 183.05889299999998, 132.626393],
    [ 187.79999999999998, 129.2],
    [ 190.0, 134.0],
    [ 192.0, 139.79999999999998],
    [ 197.64999999999998, 138.25],
    [ 201.0, 143.2],
    [ 205.15889299999998, 147.426393],
    [ 209.5, 152.9],
    [ 210.0, 158.0],
    [ 209.54999999999998, 162.45],
    [ 207.7, 167.2],
    [ 203.2, 170.4],
    [ 198.2, 170.9],
    [ 192.758893, 170.12639299999998],
    [ 187.15889299999998, 168.926393],
    [ 183.358893, 164.826393],
    [ 179.55889299999998, 160.726393],
    [ 177.658893, 156.52639299999998],
    [ 176.0, 151.7],
    [ 182.05889299999998, 150.02639299999998],
    [ 182.758893, 169.326393],
    [ 185.15, 174.25],
    [ 190.79999999999998, 175.9],
    [ 187.6, 179.79999999999998],
    [ 193.54999999999998, 182.35],
    [ 191.1, 185.79999999999998],
    [ 197.39999999999998, 187.6],
    [ 195.55889299999998, 191.62639299999998],
    [ 201.958893, 191.02639299999998],
    [ 203.65889299999998, 195.52639299999998],
    [ 209.89999999999998, 194.7],
    [ 211.85, 199.14999999999998],
    [ 217.2, 195.39999999999998],
    [ 219.2, 199.2],
    [ 222.958893, 194.726393],
    [ 228.6, 195.2],
    [ 228.758893, 190.62639299999998],
    [ 233.35, 188.54999999999998],
    [ 233.79999999999998, 194.5],
    [ 238.2, 192.35],
    [ 244.29999999999998, 192.6],
    [ 241.45, 197.25],
    [ 248.5, 199.0],
    [ 250.39999999999998, 194.29999999999998],
    [ 253.35, 200.45],
    [ 255.39999999999998, 196.0],
    [ 261.09999999999997, 200.5],
    [ 261.09999999999997, 194.89999999999998],
    [ 266.59999999999997, 197.75],
    [ 266.85889299999997, 192.326393],
    [ 263.558893, 188.926393],
    [ 267.4, 185.2],
    [ 263.258893, 182.326393],
    [ 266.258893, 178.62639299999998],
    [ 261.59999999999997, 176.29999999999998],
    [ 264.3, 172.4],
    [ 259.758893, 170.426393],
    [ 262.25, 166.54999999999998],
    [ 258.15, 164.04999999999998],
    [ 259.59999999999997, 160.4],
    [ 255.1, 158.5],
    [ 259.25, 155.85],
    [ 254.14999999999998, 154.45],
    [ 249.2, 156.15],
    [ 244.1, 154.79999999999998],
    [ 239.39999999999998, 155.2],
    [ 234.7, 154.1],
    [ 230.65889299999998, 157.726393],
    [ 229.0, 153.6],
    [ 222.89999999999998, 152.7],
    [ 227.89999999999998, 161.4],
    [ 226.75, 165.54999999999998],
    [ 224.64999999999998, 169.95],
    [ 224.89999999999998, 174.7],
    [ 226.75, 179.65],
    [ 229.258893, 185.02639299999998],
    [ 219.04999999999998, 169.75],
    [ 217.1, 165.6],
    [ 215.29999999999998, 161.79999999999998],
    [ 193.05889299999998, 127.42639299999999],
    [ 198.858893, 125.526393],
    [ 205.0, 124.5],
    [ 211.65889299999998, 123.92639299999999],
    [ 217.89999999999998, 124.6],
    [ 224.75, 127.64999999999999],
    [ 229.358893, 132.926393],
    [ 232.358893, 137.326393],
    [ 232.64999999999998, 142.04999999999998],
    [ 233.0, 148.0],
    [ 235.89999999999998, 133.5],
    [ 241.29999999999998, 136.79999999999998],
    [ 245.858893, 141.426393],
    [ 250.15889299999998, 146.226393],
    [ 254.1, 150.4],
    [ 260.7, 151.29999999999998],
    [ 266.05, 149.04999999999998],
    [ 265.758893, 144.52639299999998],
    [ 268.958893, 140.426393],
    [ 266.35889299999997, 136.326393],
    [ 266.9, 132.5],
    [ 263.2, 129.0],
    [ 262.4, 125.3],
    [ 264.4, 120.89999999999999],
    [ 263.9, 116.0],
    [ 265.35889299999997, 111.126393],
    [ 261.3, 107.626393],
    [ 260.2, 103.6],
    [ 254.7, 99.8],
    [ 251.64999999999998, 96.14999999999999],
    [ 247.29999999999998, 93.0],
    [ 230.54999999999998, 87.14999999999999],
    [ 224.89999999999998, 84.6],
    [ 220.55889299999998, 87.826393],
    [ 216.54999999999998, 84.35],
    [ 212.89999999999998, 88.5],
    [ 209.55889299999998, 84.226393],
    [ 207.358893, 87.826393],
    [ 203.79999999999998, 91.8],
    [ 198.45, 93.95],
    [ 196.2, 97.89999999999999],
    [ 191.5, 96.39999999999999],
    [ 179.0, 106.39999999999999],
    [ 177.758893, 115.326393],
    [ 183.35, 113.25],
    [ 204.758893, 113.126393],
    [ 209.958893, 116.126393],
    [ 215.858893, 120.026393],
    [ 195.55889299999998, 102.526393],
    [ 204.1, 82.7],
    [ 199.05889299999998, 82.326393],
    [ 194.1, 80.5],
    [ 208.2, 78.89999999999999],
    [ 214.29999999999998, 78.0],
    [ 208.89999999999998, 73.39999999999999],
    [ 207.79999999999998, 68.64999999999999],
    [ 205.958893, 63.726392999999995],
    [ 201.958893, 59.426393],
    [ 196.6, 57.099999999999994],
    [ 190.5, 58.9],
    [ 185.05889299999998, 62.226392999999995],
    [ 185.5, 67.0],
    [ 180.858893, 70.026393],
    [ 181.29999999999998, 75.0],
    [ 177.1, 78.8],
    [ 178.0, 83.39999999999999],
    [ 174.858893, 88.226393],
    [ 174.05889299999998, 93.126393],
    [ 172.658893, 97.92639299999999],
    [ 172.0, 103.05],
    [ 190.1, 53.699999999999996],
    [ 196.1, 51.9],
    [ 193.7, 47.9],
    [ 200.858893, 46.126393],
    [ 197.258893, 42.226393],
    [ 203.2, 41.6],
    [ 208.89999999999998, 42.5],
    [ 209.2, 38.3],
    [ 202.0, 36.65],
    [ 207.55889299999998, 32.826392999999996],
    [ 213.55889299999998, 34.726393],
    [ 212.14999999999998, 27.549999999999997],
    [ 217.55889299999998, 28.726392999999998],
    [ 217.54999999999998, 22.95],
    [ 223.54999999999998, 24.45],
    [ 224.89999999999998, 17.8],
    [ 231.39999999999998, 20.15],
    [ 231.54999999999998, 15.149999999999999],
    [ 236.79999999999998, 11.1],
    [ 238.79999999999998, 15.899999999999999],
    [ 243.1, 8.5],
    [ 245.6, 13.5],
    [ 248.258893, 5.8263929999999995],
    [ 251.6, 11.45],
    [ 255.54999999999998, 4.85],
    [ 257.3, 9.2],
    [ 263.35889299999997, 8.426393],
    [ 262.09999999999997, 3.6999999999999997],
    [ 267.0, 2.1],
    [ 269.0, 7.199999999999999],
    [ 271.7, 2.5],
    [ 308.658893, 14.426393],
    [ 309.15, 7.6499999999999995],
    [ 304.85889299999997, 17.526393],
    [ 313.058893, 15.45],
    [ 312.45, 19.45],
    [ 309.95, 23.55],
    [ 304.55, 21.55],
    [ 300.05, 21.75],
    [ 303.84999999999997, 25.95],
    [ 295.25, 24.049999999999997],
    [ 294.05, 28.049999999999997],
    [ 292.45, 31.849999999999998],
    [ 298.75, 29.65],
    [ 299.55, 33.65],
    [ 299.84999999999997, 37.75],
    [ 290.15, 36.25],
    [ 288.55, 40.55],
    [ 294.25, 39.75],
    [ 298.75, 41.85],
    [ 294.65, 44.75],
    [ 287.65, 45.55],
    [ 285.75, 48.75],
    [ 288.558893, 51.926393],
    [ 287.558893, 56.726392999999995],
    [ 280.35889299999997, 52.926393],
    [ 280.158893, 56.426393],
    [ 282.9, 59.4],
    [ 274.9, 55.5],
    [ 278.758893, 60.199999999999996],
    [ 274.4, 62.3],
    [ 270.0, 60.099999999999994],
    [ 269.09999999999997, 56.4],
    [ 263.9, 55.4],
    [ 259.09999999999997, 54.199999999999996],
    [ 254.1, 53.699999999999996],
    [ 249.6, 53.099999999999994],
    [ 244.89999999999998, 53.8],
    [ 240.0, 54.3],
    [ 235.758893, 54.726392999999995],
    [ 230.258893, 55.026393],
    [ 225.358893, 50.526393],
    [ 224.7, 47.1],
    [ 219.2, 44.699999999999996],
    [ 215.55889299999998, 40.826392999999996],
    [ 224.858893, 56.326392999999996],
    [ 223.29999999999998, 61.5],
    [ 221.5, 66.6],
    [ 218.35, 72.55],
    [ 234.79999999999998, 84.3],
    [ 238.05889299999998, 89.326393],
    [ 240.7, 84.3],
    [ 243.39999999999998, 89.8],
    [ 246.55889299999998, 83.92639299999999],
    [ 252.15889299999998, 87.726393],
    [ 254.358893, 83.726393],
    [ 259.0, 87.39999999999999],
    [ 260.658893, 83.126393],
    [ 263.9, 86.39999999999999],
    [ 270.058893, 82.326393],
    [ 265.5, 82.326393],
    [ 270.85889299999997, 78.226393],
    [ 274.2, 81.39999999999999],
    [ 276.458893, 78.326393],
    [ 277.058893, 74.92639299999999],
    [ 272.958893, 74.626393],
    [ 273.258893, 70.826393],
    [ 277.55, 70.75],
    [ 276.84999999999997, 67.14999999999999],
    [ 272.95, 66.45],
    ];
const port : [[f32;2];256] = [
    [ 127.94110700000002, 117.226393],
    [ 126.9, 112.39999999999999],
    [ 127.641107, 121.626393],
    [ 128.241107, 126.42639299999999],
    [ 128.041107, 131.126393],
    [ 122.341107, 128.426393],
    [ 123.80000000000001, 132.6],
    [ 116.94110700000002, 132.626393],
    [ 112.20000000000002, 129.2],
    [ 110.0, 134.0],
    [ 108.0, 139.79999999999998],
    [ 102.35000000000002, 138.25],
    [ 99.0, 143.2],
    [ 94.84110700000002, 147.426393],
    [ 90.5, 152.9],
    [ 90.0, 158.0],
    [ 90.45000000000002, 162.45],
    [ 92.30000000000001, 167.2],
    [ 96.80000000000001, 170.4],
    [ 101.80000000000001, 170.9],
    [ 107.241107, 170.12639299999998],
    [ 112.84110700000002, 168.926393],
    [ 116.641107, 164.826393],
    [ 120.44110700000002, 160.726393],
    [ 122.341107, 156.52639299999998],
    [ 124.0, 151.7],
    [ 117.94110700000002, 150.02639299999998],
    [ 117.241107, 169.326393],
    [ 114.85, 174.25],
    [ 109.20000000000002, 175.9],
    [ 112.4, 179.79999999999998],
    [ 106.45000000000002, 182.35],
    [ 108.9, 185.79999999999998],
    [ 102.60000000000002, 187.6],
    [ 104.44110700000002, 191.62639299999998],
    [ 98.04110700000001, 191.02639299999998],
    [ 96.34110700000002, 195.52639299999998],
    [ 91.10000000000002, 194.89999999999998],
    [ 88.15, 199.14999999999998],
    [ 82.80000000000001, 195.39999999999998],
    [ 80.80000000000001, 199.2],
    [ 77.04110700000001, 194.726393],
    [ 71.4, 195.2],
    [ 71.241107, 190.62639299999998],
    [ 66.65, 188.54999999999998],
    [ 66.20000000000002, 194.5],
    [ 61.80000000000001, 192.35],
    [ 55.70000000000002, 192.6],
    [ 58.55000000000001, 197.25],
    [ 51.5, 199.0],
    [ 49.60000000000002, 194.29999999999998],
    [ 46.650000000000006, 200.45],
    [ 44.60000000000002, 196.0],
    [ 38.900000000000034, 200.5],
    [ 38.900000000000034, 194.89999999999998],
    [ 33.400000000000034, 197.75],
    [ 33.141107000000034, 192.326393],
    [ 36.44110699999999, 188.926393],
    [ 32.60000000000002, 185.2],
    [ 36.741107, 182.326393],
    [ 33.741107, 178.62639299999998],
    [ 38.400000000000034, 176.29999999999998],
    [ 35.69999999999999, 172.4],
    [ 40.241107, 170.426393],
    [ 37.75, 166.54999999999998],
    [ 41.85000000000002, 164.04999999999998],
    [ 40.400000000000034, 160.4],
    [ 44.900000000000006, 158.5],
    [ 40.75, 155.85],
    [ 45.85000000000002, 154.45],
    [ 50.80000000000001, 156.15],
    [ 55.900000000000006, 154.79999999999998],
    [ 60.60000000000002, 155.2],
    [ 65.30000000000001, 154.1],
    [ 69.34110700000002, 157.726393],
    [ 71.0, 153.6],
    [ 77.10000000000002, 152.7],
    [ 72.10000000000002, 161.4],
    [ 73.25, 165.54999999999998],
    [ 75.35000000000002, 169.95],
    [ 75.10000000000002, 174.7],
    [ 73.25, 179.65],
    [ 70.741107, 185.02639299999998],
    [ 80.95000000000002, 169.75],
    [ 82.9, 165.6],
    [ 84.70000000000002, 161.79999999999998],
    [ 106.94110700000002, 127.42639299999999],
    [ 101.141107, 125.526393],
    [ 95.0, 124.5],
    [ 88.34110700000002, 123.92639299999999],
    [ 82.10000000000002, 124.6],
    [ 75.25, 127.64999999999999],
    [ 70.641107, 132.926393],
    [ 67.641107, 137.326393],
    [ 67.35000000000002, 142.04999999999998],
    [ 67.0, 148.0],
    [ 64.10000000000002, 133.5],
    [ 58.70000000000002, 136.79999999999998],
    [ 54.141107000000005, 141.426393],
    [ 49.84110700000002, 146.226393],
    [ 45.900000000000006, 150.4],
    [ 39.30000000000001, 151.29999999999998],
    [ 33.94999999999999, 149.04999999999998],
    [ 34.241107, 144.52639299999998],
    [ 31.04110700000001, 140.426393],
    [ 33.641107000000034, 136.326393],
    [ 33.10000000000002, 132.5],
    [ 36.80000000000001, 129.0],
    [ 37.60000000000002, 125.3],
    [ 36.10000000000002, 121.5],
    [ 35.80000000000001, 117.39999999999999],
    [ 35.5, 113.05],
    [ 37.30000000000001, 109.39999999999999],
    [ 39.0, 105.69999999999999],
    [ 42.85000000000002, 101.94999999999999],
    [ 45.0, 98.5],
    [ 49.20000000000002, 95.6],
    [ 69.45000000000002, 87.14999999999999],
    [ 75.10000000000002, 84.6],
    [ 79.44110700000002, 87.826393],
    [ 83.45000000000002, 84.35],
    [ 87.10000000000002, 88.5],
    [ 90.44110700000002, 84.226393],
    [ 92.641107, 87.826393],
    [ 96.20000000000002, 91.8],
    [ 101.55000000000001, 93.95],
    [ 103.80000000000001, 97.89999999999999],
    [ 108.5, 96.39999999999999],
    [ 121.0, 106.39999999999999],
    [ 122.241107, 115.326393],
    [ 116.65, 113.25],
    [ 95.241107, 113.126393],
    [ 90.04110700000001, 116.126393],
    [ 84.141107, 120.026393],
    [ 103.9, 102.89999999999999],
    [ 95.9, 82.7],
    [ 100.94110700000002, 82.326393],
    [ 105.9, 80.5],
    [ 91.80000000000001, 78.89999999999999],
    [ 85.70000000000002, 78.0],
    [ 91.10000000000002, 73.39999999999999],
    [ 92.20000000000002, 68.64999999999999],
    [ 94.04110700000001, 63.726392999999995],
    [ 98.04110700000001, 59.426393],
    [ 103.4, 57.099999999999994],
    [ 109.5, 58.9],
    [ 114.94110700000002, 62.226392999999995],
    [ 114.5, 67.0],
    [ 119.141107, 70.026393],
    [ 118.70000000000002, 75.0],
    [ 122.9, 78.8],
    [ 122.0, 83.39999999999999],
    [ 125.141107, 88.226393],
    [ 125.94110700000002, 93.126393],
    [ 127.341107, 97.92639299999999],
    [ 128.0, 103.05],
    [ 109.9, 53.699999999999996],
    [ 103.9, 51.9],
    [ 106.30000000000001, 47.9],
    [ 99.141107, 46.126393],
    [ 102.741107, 42.226393],
    [ 96.80000000000001, 41.6],
    [ 91.10000000000002, 42.5],
    [ 90.80000000000001, 38.3],
    [ 98.0, 36.65],
    [ 92.44110700000002, 32.826392999999996],
    [ 86.44110700000002, 34.726393],
    [ 87.85000000000002, 27.549999999999997],
    [ 82.44110700000002, 28.726392999999998],
    [ 82.45000000000002, 22.95],
    [ 76.45000000000002, 24.45],
    [ 75.10000000000002, 17.8],
    [ 68.60000000000002, 20.15],
    [ 68.45000000000002, 15.149999999999999],
    [ 63.20000000000002, 11.1],
    [ 61.20000000000002, 15.899999999999999],
    [ 56.900000000000006, 8.5],
    [ 54.400000000000006, 13.5],
    [ 51.741107, 5.8263929999999995],
    [ 48.400000000000006, 11.45],
    [ 44.45000000000002, 4.85],
    [ 42.69999999999999, 9.2],
    [ 36.641107000000034, 8.426393],
    [ 37.900000000000034, 3.6999999999999997],
    [ 33.10000000000002, 3.4],
    [ 31.0, 7.199999999999999],
    [ 28.30000000000001, 2.5],
    [ -6.599999999999966, 12.1],
    [ -9.149999999999977, 7.6499999999999995],
    [ -7.399999999999977, 17.099999999999998],
    [ -13.058893000000012, 15.45],
    [ -12.449999999999989, 19.45],
    [ -9.949999999999989, 23.55],
    [ -4.550000000000011, 21.55],
    [ -0.05000000000001137, 21.75],
    [ -3.849999999999966, 25.95],
    [ 4.75, 24.049999999999997],
    [ 5.949999999999989, 28.049999999999997],
    [ 7.550000000000011, 31.849999999999998],
    [ 1.25, 29.65],
    [ 0.44999999999998863, 33.65],
    [ 0.1500000000000341, 37.75],
    [ 7.300000000000011, 36.1],
    [ 12.341107000000022, 37.673607],
    [ 5.75, 39.75],
    [ 1.25, 41.85],
    [ 5.350000000000023, 44.75],
    [ 12.04110700000001, 43.573606999999996],
    [ 14.25, 48.75],
    [ 11.441106999999988, 51.926393],
    [ 12.441106999999988, 56.726392999999995],
    [ 16.900000000000034, 53.0],
    [ 19.841107000000022, 56.426393],
    [ 17.100000000000023, 59.4],
    [ 25.100000000000023, 55.5],
    [ 21.241107, 60.199999999999996],
    [ 25.600000000000023, 62.3],
    [ 30.0, 60.099999999999994],
    [ 30.900000000000034, 56.4],
    [ 36.10000000000002, 55.4],
    [ 40.900000000000034, 54.199999999999996],
    [ 45.900000000000006, 53.699999999999996],
    [ 50.400000000000006, 53.099999999999994],
    [ 55.10000000000002, 53.8],
    [ 60.0, 54.3],
    [ 64.241107, 54.726392999999995],
    [ 69.741107, 55.026393],
    [ 74.641107, 50.526393],
    [ 75.30000000000001, 47.1],
    [ 80.80000000000001, 44.699999999999996],
    [ 84.44110700000002, 40.826392999999996],
    [ 75.141107, 56.326392999999996],
    [ 76.70000000000002, 61.5],
    [ 78.5, 66.6],
    [ 81.65, 72.55],
    [ 65.20000000000002, 84.3],
    [ 61.94110700000002, 89.326393],
    [ 59.30000000000001, 84.3],
    [ 53.900000000000006, 91.89999999999999],
    [ 54.400000000000006, 87.7],
    [ 52.20000000000002, 83.89999999999999],
    [ 48.44110700000002, 87.37360699999999],
    [ 45.85000000000002, 83.55],
    [ 42.400000000000034, 86.89999999999999],
    [ 39.650000000000034, 83.14999999999999],
    [ 34.0, 82.326393],
    [ 36.60000000000002, 86.39999999999999],
    [ 30.80000000000001, 78.7],
    [ 26.69999999999999, 82.8],
    [ 23.55000000000001, 79.45],
    [ 22.100000000000023, 75.25],
    [ 26.400000000000034, 75.5],
    [ 26.741107, 70.826393],
    [ 22.44999999999999, 70.75],
    [ 23.150000000000034, 67.14999999999999],
    [ 27.400000000000034, 65.39999999999999],
];
