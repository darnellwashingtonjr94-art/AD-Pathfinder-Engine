import time

def run_benchmark():
    start = time.time()
    print("Running benchmark simulation...")
    time.sleep(0.1)
    end = time.time()
    print(f"Benchmark finished in {end - start:.4f} seconds.")

if __name__ == "__main__":
    run_benchmark()
