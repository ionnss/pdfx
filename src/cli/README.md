# Init command
pdfx init                    # Indexes current directory
pdfx init ~                  # Indexes home directory
pdfx init ~/Documents        # Indexes specific directory


# Search command  
pdfx search "machine learning"  # Required argument 

# Recent command
pdfx recent                  # Uses default limit=10
pdfx recent -l 5            # Short flag
pdfx recent --limit 20      # Long flag

# List command
pdfx list                   # all=false (default for bool)
pdfx list -a               # Short flag, all=true
pdfx list --all            # Long flag, all=true