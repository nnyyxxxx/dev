#!/bin/sh -e

. ./utility_functions.sh

# Function to scale smaller monitors to the highest resolution of a bigger monitor
scale_monitors() {
    echo "${BLUE}=========================================${RESET}"
    echo "${BLUE}  Scale Monitors to Highest Resolution${RESET}"
    echo "${BLUE}=========================================${RESET}"

    monitor_list=$(detect_connected_monitors)
    IFS=$'\n' read -r -a monitor_array <<<"$monitor_list"

    # Get the highest resolution among all monitors
    max_width=0
    max_height=0

    for monitor in "${monitor_array[@]}"; do
        res=$(xrandr | grep -A1 "^$monitor connected" | tail -1 | awk '{print $1}')
        width=$(echo $res | awk -Fx '{print $1}')
        height=$(echo $res | awk -Fx '{print $2}')

        if (( width > max_width )); then
            max_width=$width
        fi

        if (( height > max_height )); then
            max_height=$height
        fi
    done

    echo "${CYAN}Highest resolution found: ${max_width}x${max_height}${RESET}"

    # Scale all monitors to the maximum resolution
    for monitor in "${monitor_array[@]}"; do
        echo "${CYAN}Scaling $monitor to ${max_width}x${max_height}${RESET}"
        execute_command "xrandr --output $monitor --scale-from ${max_width}x${max_height}"
    done

    echo "${GREEN}Scaling complete. All monitors are now scaled to ${max_width}x${max_height}.${RESET}"
}

# Call the scale_monitors function
scale_monitors
