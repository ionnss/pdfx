# Init command
pdfx init                    # Indexes current directory
pdfx init ~                  # Indexes home directory
pdfx init ~/Documents        # Indexes specific directory

# Search command  
pdfx search "machine learning"  # Required argument
pdfx search "rust programming" --filename   # Search in filenames only
pdfx search "concurrency" --content         # Search in PDF content only
pdfx search "async"                         # Search in both (default)

# List command
pdfx list                   # all=false (default for bool)
pdfx list -a                # Short flag, all=true
pdfx list --all             # Long flag, all=true

# Cleanup command
pdfx cleanup                # Cleans all local pdfx indexed data