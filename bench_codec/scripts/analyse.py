import pandas as pd
import matplotlib.pyplot as plt

path = "path/to/result.csv"
df = pd.read_csv(path)
df = df[df['codec'].isin(['plain + zstd', 'delta + zstd'])]

# Plotting
fig, axs = plt.subplots(2, 1, figsize=(12, 18))

# Compression time comparison
# axs[0].bar(df['codec'], df['compress_time_ms'], color='b')
# axs[0].set_title('Compression Time (ms)')
# axs[0].set_ylabel('Time (ms)')
# axs[0].set_xticklabels(df['codec'], rotation=45, ha='right')

# Compressed size comparison
axs[0].bar(df['codec'], df['compressed_kb'], color='g')
axs[0].set_title('Compressed Size (KB)')
axs[0].set_ylabel('Size (KB)')
axs[0].set_xticklabels(df['codec'], rotation=45, ha='right')

# Decompression time comparison
axs[1].bar(df['codec'], df['decompress_time_ms'], color='r')
axs[1].set_title('Decompression Time (ms)')
axs[1].set_ylabel('Time (ms)')
axs[1].set_xticklabels(df['codec'], rotation=45, ha='right')

plt.tight_layout()
plt.savefig('customer.c_name.png')
