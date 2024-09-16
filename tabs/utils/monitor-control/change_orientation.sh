#!/bin/sh -e

. ./utility_functions.sh

# Function to change monitor orientation
change_orientation() {
    monitor_list=$(detect_connected_monitors)
    IFS=$'\n' read -r -a monitor_array <<<"$monitor_list"

    clear
    echo "${BLUE}=========================================${RESET}"
    echo "${BLUE}  Change Monitor Orientation${RESET}"
    echo "${BLUE}=========================================${RESET}"
    echo "${YELLOW}Choose a monitor to configure:${RESET}"
    for i in "${!monitor_array[@]}"; do
        echo "$((i + 1)). ${CYAN}${monitor_array[i]}${RESET}"
    done

    read -p "Enter the number of the monitor: " monitor_choice

    if ! [[ "$monitor_choice" =~ ^[0-9]+$ ]] || (( monitor_choice < 1 )) || (( monitor_choice > ${#monitor_array[@]} )); then
        echo "${RED}Invalid selection.${RESET}"
        return
    fi

    monitor_name="${monitor_array[monitor_choice - 1]}"

    clear
    echo "${BLUE}=========================================${RESET}"
    echo "${BLUE}  Set Orientation for $monitor_name${RESET}"
    echo "${BLUE}=========================================${RESET}"
    echo "${YELLOW}Choose orientation:${RESET}"
    echo "1. ${CYAN}Normal${RESET}"
    echo "2. ${CYAN}Left${RESET}"
    echo "3. ${CYAN}Right${RESET}"
    echo "4. ${CYAN}Inverted${RESET}"

    read -p "Enter the number of the orientation: " orientation_choice

    case $orientation_choice in
        1) orientation="normal" ;;
        2) orientation="left" ;;
        3) orientation="right" ;;
        4) orientation="inverted" ;;
        *) echo "${RED}Invalid selection.${RESET}"; return ;;
    esac

    if confirm_action "Change orientation of ${CYAN}$monitor_name${RESET} to ${CYAN}$orientation${RESET}?"; then
        echo "${GREEN}Changing orientation of $monitor_name to $orientation${RESET}"
        execute_command "xrandr --output $monitor_name --rotate $orientation"
        echo "${GREEN}Orientation changed successfully.${RESET}"
    else
        echo "${RED}Action canceled.${RESET}"
    fi
}

# Call the change_orientation function
change_orientation
